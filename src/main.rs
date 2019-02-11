use std::slice;
use std::marker::PhantomData;

struct SliceField<'a> {
    ptr: *mut i32,
    len: usize,
    field: usize,
    marker: PhantomData<&'a mut i32>,
}

impl SliceField<'_> {
    fn inc(&mut self) {
        let slice = unsafe { slice::from_raw_parts_mut(self.ptr, self.len) };
        for i in (self.field..self.len).step_by(2) {
            slice[i] += 1;
        }
    }

    fn dec(&mut self) {
        let slice = unsafe { slice::from_raw_parts_mut(self.ptr, self.len) };
        for i in (self.field..self.len).step_by(2) {
            slice[i] += 1;
        }
    }
}

unsafe impl Send for SliceField<'_> {}

fn split_fields(array: &mut [i32]) -> (SliceField<'_>, SliceField<'_>) {
    (
        SliceField {
            ptr: array.as_mut_ptr(),
            len: array.len(),
            field: 0,
            marker: PhantomData,
        },
        SliceField {
            ptr: array.as_mut_ptr(),
            len: array.len(),
            field: 1,
            marker: PhantomData,
        },
    )
}

fn main() {
    let data = &mut [0i32; 11];
    {
        let (mut even, mut odd) = split_fields(data);
        rayon::join(|| even.inc(), || odd.dec());
    }
    println!("{:?}", data);
}
