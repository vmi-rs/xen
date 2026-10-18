use xen_sys::{
    __HVM_SAVE_TYPE_CPU, __HVM_SAVE_TYPE_END, __HVM_SAVE_TYPE_HEADER, __HVM_SAVE_TYPE_HPET,
    __HVM_SAVE_TYPE_LAPIC, __HVM_SAVE_TYPE_LAPIC_REGS, __HVM_SAVE_TYPE_PMTIMER,
    __HVM_SAVE_TYPE_RTC, __HVM_SAVE_TYPE_VIRIDIAN_DOMAIN, __HVM_SAVE_TYPE_VIRIDIAN_VCPU,
    hvm_hw_cpu, hvm_hw_hpet, hvm_hw_lapic, hvm_hw_lapic_regs, hvm_hw_pmtimer, hvm_hw_rtc,
    hvm_save_descriptor, hvm_save_header, hvm_viridian_domain_context, hvm_viridian_vcpu_context,
};

/// A part of a domain's HVM context.
///
/// Xen saves the state of a domain's virtual hardware as a list of records, one
/// record per part: a vCPU's registers, its local APIC, the RTC, the HPET and
/// so on. Each kind of record is a C struct from Xen's headers, such as
/// `hvm_hw_cpu`. libxen implements this trait for those structs only, because
/// it reads a record's raw bytes as the struct, and that is safe only for plain
/// C structs.
pub trait XenHvmSaveRecord: sealed::Sealed + Copy {
    /// Returns the code of this kind of record.
    ///
    /// Every record in an HVM context starts with a small header, and the code
    /// in that header says which kind of record follows. Xen's C headers call
    /// it `HVM_SAVE_CODE`.
    fn typecode() -> u16;
}

mod sealed {
    /// Keeps other crates from implementing `XenHvmSaveRecord`.
    pub trait Sealed {}
}

macro_rules! hvm_save_record {
    ($($record:ty => $save_type:ty),* $(,)?) => {
        $(
            impl sealed::Sealed for $record {}

            impl XenHvmSaveRecord for $record {
                fn typecode() -> u16 {
                    // Same as Xen's `HVM_SAVE_CODE(x)`, which is the `sizeof`
                    // of the `c` member of `struct __HVM_SAVE_TYPE_x`.
                    size_of_val(&<$save_type>::default().c) as u16
                }
            }
        )*
    };
}

hvm_save_record! {
    hvm_save_header => __HVM_SAVE_TYPE_HEADER,
    hvm_hw_cpu => __HVM_SAVE_TYPE_CPU,
    hvm_hw_lapic => __HVM_SAVE_TYPE_LAPIC,
    hvm_hw_lapic_regs => __HVM_SAVE_TYPE_LAPIC_REGS,
    hvm_hw_rtc => __HVM_SAVE_TYPE_RTC,
    hvm_hw_hpet => __HVM_SAVE_TYPE_HPET,
    hvm_hw_pmtimer => __HVM_SAVE_TYPE_PMTIMER,
    hvm_viridian_domain_context => __HVM_SAVE_TYPE_VIRIDIAN_DOMAIN,
    hvm_viridian_vcpu_context => __HVM_SAVE_TYPE_VIRIDIAN_VCPU,
}

/// A domain's HVM context.
///
/// Xen saves an HVM domain's virtual hardware as a list of records: the
/// registers of each vCPU, its local APIC, and the RTC, HPET and other timers.
/// Xen uses the same records to save, restore and migrate a domain. `get` reads
/// one record, and `update` changes records before `XenDomain::set_context`
/// loads them back.
#[derive(Debug, Clone)]
pub struct XenHvmContext {
    /// The raw records.
    ///
    /// Each record is a `hvm_save_descriptor` header followed by the record's
    /// data. A record of type 0 ends the list.
    buffer: Vec<u8>,
}

/// The position and kind of one record in the context.
///
/// It holds what the record's `hvm_save_descriptor` header says, plus the
/// offset at which the record's data starts.
#[derive(Debug, Clone, Copy)]
struct Descriptor {
    /// The kind of record, as `XenHvmSaveRecord::typecode` returns it.
    typecode: u16,

    /// Which copy of the record this is.
    ///
    /// Records such as `hvm_hw_cpu` exist once per vCPU, and their instance is
    /// the vCPU's id. A record of the whole domain, such as the RTC's, has
    /// instance 0.
    instance: u16,

    /// The offset in bytes of the record's data in the buffer. The data starts
    /// right after the record's header.
    data: usize,

    /// The length in bytes of the record's data, without its header. A record
    /// can be shorter or longer than its Rust struct. `read` checks the length
    /// before it copies the record.
    length: usize,
}

impl XenHvmContext {
    /// Creates a context from the bytes that `xc_domain_hvm_getcontext` wrote.
    ///
    /// The bytes stay as they are. `XenDomain::set_context` hands them back to
    /// Xen.
    pub(crate) fn new(buffer: Vec<u8>) -> Self {
        Self { buffer }
    }

    /// Returns the context's bytes.
    ///
    /// `XenDomain::set_context` passes them to Xen unchanged, with the records
    /// that `update` changed.
    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }

    /// Returns a copy of one record.
    ///
    /// `T` picks the kind of record, such as `hvm_hw_cpu` for a vCPU's
    /// registers. `instance` picks the vCPU, or is 0 for a record of the whole
    /// domain. Returns `None` if the context has no such record or the record
    /// is too short.
    pub fn get<T>(&self, instance: u16) -> Option<T>
    where
        T: XenHvmSaveRecord,
    {
        let typecode = T::typecode();
        self.descriptors()
            .find(|descriptor| descriptor.typecode == typecode && descriptor.instance == instance)
            .and_then(|descriptor| self.read(descriptor))
    }

    /// Changes the records of one kind in place.
    ///
    /// `f` is called once for each record of type `T`, with the record's
    /// instance and a copy of the record. What `f` changes in the copy is
    /// written back into the context. `XenDomain::set_context` then loads the
    /// change into the domain.
    pub fn update<T>(&mut self, mut f: impl FnMut(u16, &mut T))
    where
        T: XenHvmSaveRecord,
    {
        let typecode = T::typecode();
        let descriptors = self
            .descriptors()
            .filter(|descriptor| descriptor.typecode == typecode)
            .collect::<Vec<_>>();

        for descriptor in descriptors {
            if let Some(mut record) = self.read::<T>(descriptor) {
                f(descriptor.instance, &mut record);
                self.write(descriptor, record);
            }
        }
    }

    /// Returns the records in the order Xen wrote them.
    ///
    /// The walk stops at the end record. A record that does not fit in the
    /// buffer ends the walk early.
    fn descriptors(&self) -> impl Iterator<Item = Descriptor> + '_ {
        let end = size_of_val(&__HVM_SAVE_TYPE_END::default().c) as u16;
        let mut offset = 0;

        std::iter::from_fn(move || {
            let data = offset + size_of::<hvm_save_descriptor>();
            if data > self.buffer.len() {
                return None;
            }

            // SAFETY: This reads `size_of::<hvm_save_descriptor>()` bytes from
            // `offset`. The check above made sure that those bytes are in the
            // buffer, and `read_unaligned` does not care where they start.
            let descriptor = unsafe {
                std::ptr::read_unaligned(
                    self.buffer.as_ptr().add(offset) as *const hvm_save_descriptor
                )
            };

            let length = descriptor.length as usize;
            if descriptor.typecode == end || data + length > self.buffer.len() {
                return None;
            }

            offset = data + length;
            Some(Descriptor {
                typecode: descriptor.typecode,
                instance: descriptor.instance,
                data,
                length,
            })
        })
    }

    /// Copies a record's data into a `T`.
    ///
    /// Returns `None` if the record holds fewer bytes than `T`. A record that
    /// holds more gives its first `size_of::<T>()` bytes.
    fn read<T>(&self, descriptor: Descriptor) -> Option<T>
    where
        T: XenHvmSaveRecord,
    {
        if descriptor.length < size_of::<T>() {
            return None;
        }

        // SAFETY: `descriptors` keeps every record within the buffer, and the
        // check above makes sure that this one has at least `size_of::<T>()`
        // bytes. The sealed trait limits `T` to Xen's record structs. These
        // structs hold only integers. Any bytes therefore make a valid `T`.
        // `read_unaligned` does not need an aligned buffer.
        Some(unsafe {
            std::ptr::read_unaligned(self.buffer.as_ptr().add(descriptor.data) as *const T)
        })
    }

    /// Writes a changed record back into the buffer.
    ///
    /// `read` must have accepted `descriptor` for the same `T`, so the record
    /// is known to have room for it.
    fn write<T>(&mut self, descriptor: Descriptor, record: T)
    where
        T: XenHvmSaveRecord,
    {
        // SAFETY: This writes `size_of::<T>()` bytes at `descriptor.data`, the
        // same bytes that `read` checked and copied.
        unsafe {
            std::ptr::write_unaligned(
                self.buffer.as_mut_ptr().add(descriptor.data) as *mut T,
                record,
            )
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Appends a record to a test buffer.
    ///
    /// It writes a header with `T`'s type code, `instance` and `T`'s size, then
    /// the record's bytes. That is the layout Xen uses.
    fn push<T: XenHvmSaveRecord>(buffer: &mut Vec<u8>, instance: u16, record: T) {
        let descriptor = hvm_save_descriptor {
            typecode: T::typecode(),
            instance,
            length: size_of::<T>() as u32,
        };
        let bytes = |value: *const u8, size| unsafe { std::slice::from_raw_parts(value, size) };
        buffer.extend_from_slice(bytes(
            &descriptor as *const _ as *const u8,
            size_of::<hvm_save_descriptor>(),
        ));
        buffer.extend_from_slice(bytes(&record as *const T as *const u8, size_of::<T>()));
    }

    #[test]
    fn records_are_found_and_updated_by_type_and_instance() {
        let mut buffer = Vec::new();
        push(
            &mut buffer,
            0,
            hvm_save_header {
                gtsc_khz: 1_996_800,
                ..Default::default()
            },
        );
        for vcpu in 0..2 {
            push(
                &mut buffer,
                vcpu,
                hvm_hw_cpu {
                    tsc: 1000 + u64::from(vcpu),
                    ..Default::default()
                },
            );
        }
        push(
            &mut buffer,
            0,
            hvm_hw_pmtimer {
                tmr_val: 7,
                ..Default::default()
            },
        );
        buffer.extend_from_slice(&[0; size_of::<hvm_save_descriptor>()]);
        // Adds 16 bytes of garbage after the end record, which the walk must
        // not read.
        buffer.extend_from_slice(&[0xff; 16]);

        let mut context = XenHvmContext::new(buffer);
        context.update(|instance, cpu: &mut hvm_hw_cpu| cpu.tsc += 10 * u64::from(instance + 1));

        assert_eq!(
            context
                .get::<hvm_save_header>(0)
                .map(|header| header.gtsc_khz),
            Some(1_996_800)
        );
        assert_eq!(context.get::<hvm_hw_cpu>(0).map(|cpu| cpu.tsc), Some(1010));
        assert_eq!(context.get::<hvm_hw_cpu>(1).map(|cpu| cpu.tsc), Some(1021));
        assert_eq!(context.get::<hvm_hw_cpu>(2).map(|cpu| cpu.tsc), None);
        assert_eq!(
            context.get::<hvm_hw_pmtimer>(0).map(|timer| timer.tmr_val),
            Some(7)
        );
        assert!(context.get::<hvm_hw_hpet>(0).is_none());
    }
}
