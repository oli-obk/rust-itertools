extern crate test;
extern crate itertools;

use itertools::Stride;

use itertools::{Zip, ZipTrusted};

use std::iter::repeat;

#[bench]
fn slice_iter(b: &mut test::Bencher)
{
    let xs: Vec<_> = repeat(1i32).take(20).collect();
    b.iter(|| for elt in xs.as_slice().iter() {
        test::black_box(elt);
    })
}

#[bench]
fn slice_iter_rev(b: &mut test::Bencher)
{
    let xs: Vec<_> = repeat(1i32).take(20).collect();
    b.iter(|| for elt in xs.as_slice().iter().rev() {
        test::black_box(elt);
    })
}

#[bench]
fn stride_iter(b: &mut test::Bencher)
{
    let xs: Vec<_> = repeat(1i32).take(20).collect();
    b.iter(|| for elt in Stride::from_slice(xs.as_slice(), 1) {
        test::black_box(elt);
    })
}

#[bench]
fn stride_iter_rev(b: &mut test::Bencher)
{
    let xs: Vec<_> = repeat(1i32).take(20).collect();
    b.iter(|| for elt in Stride::from_slice(xs.as_slice(), 1).rev() {
        test::black_box(elt);
    })
}

#[derive(Copy)]
struct ZipSlices<'a, T, U>
{
    t_ptr: *const T,
    t_end: *const T,
    u_ptr: *const U,
    mark: ::std::marker::ContravariantLifetime<'a>,
}

impl<'a, T, U> ZipSlices<'a, T, U>
{
    pub fn new(t: &'a [T], u: &'a [U]) -> Self
    {
        assert!(::std::mem::size_of::<T>() != 0);
        assert!(::std::mem::size_of::<U>() != 0);
        let minl = std::cmp::min(t.len(), u.len());
        let end_ptr = unsafe {
            t.as_ptr().offset(minl as isize)
        };
        ZipSlices {
            t_ptr: t.as_ptr(),
            t_end: end_ptr,
            u_ptr: u.as_ptr(),
            mark: ::std::marker::ContravariantLifetime,
        }
    }
}

impl<'a, T, U> Iterator for ZipSlices<'a, T, U>
{
    type Item = (&'a T, &'a U);

    #[inline]
    fn next(&mut self) -> Option<(&'a T, &'a U)>
    {
        if self.t_ptr == self.t_end {
            return None
        }
        let t_elt: &T;
        let u_elt: &U;
        unsafe {
            t_elt = ::std::mem::transmute(self.t_ptr);
            self.t_ptr = self.t_ptr.offset(1);
            u_elt = ::std::mem::transmute(self.u_ptr);
            self.u_ptr = self.u_ptr.offset(1);
        }
        Some((t_elt, u_elt))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>)
    {
        let len = self.t_end as usize - self.t_ptr as usize;
        (len, Some(len))
    }
}

#[bench]
fn zip_slices_default_zip(b: &mut test::Bencher)
{
    let xs = vec![0; 1024];
    let ys = vec![0; 768];

    b.iter(|| for (&x, &y) in xs.iter().zip(ys.iter()) {
        test::black_box(x);
        test::black_box(y);
    })
}

#[bench]
fn zip_slices_default_zip3(b: &mut test::Bencher)
{
    let xs = vec![0; 1024];
    let ys = vec![0; 768];
    let zs = vec![0; 766];

    b.iter(|| for ((&x, &y), &z) in xs.iter().zip(ys.iter()).zip(zs.iter()) {
        test::black_box(x);
        test::black_box(y);
        test::black_box(z);
    })
}

#[bench]
fn zip_slices_ziptuple(b: &mut test::Bencher)
{
    let xs = vec![0; 1024];
    let ys = vec![0; 768];

    b.iter(|| for (&x, &y) in Zip::new((xs.iter(), ys.iter())) {
        test::black_box(x);
        test::black_box(y);
    })
}

#[bench]
fn zipslices(b: &mut test::Bencher)
{
    let xs = vec![0; 1024];
    let ys = vec![0; 768];

    b.iter(|| for (&x, &y) in ZipSlices::new(&xs, &ys) {
        test::black_box(x);
        test::black_box(y);
    })
}

#[bench]
fn ziptrusted(b: &mut test::Bencher)
{
    let xs = vec![0; 1024];
    let ys = vec![0; 768];

    b.iter(|| for (&x, &y) in ZipTrusted::new((xs.iter(), ys.iter())) {
        test::black_box(x);
        test::black_box(y);
    })
}

#[bench]
fn ziptrusted3(b: &mut test::Bencher)
{
    let xs = vec![0; 1024];
    let ys = vec![0; 768];
    let zs = vec![0; 766];

    b.iter(|| for (&x, &y, &z) in ZipTrusted::new((xs.iter(), ys.iter(), zs.iter())) {
        test::black_box(x);
        test::black_box(y);
        test::black_box(z);
    })
}

#[bench]
fn zip_loop(b: &mut test::Bencher)
{
    let xs = vec![0; 1024];
    let ys = vec![0; 768];

    b.iter(|| {
        let len = ::std::cmp::min(xs.len(), ys.len());
        for i in 0..len {
            unsafe {
            let x = *xs.get_unchecked(i);
            let y = *ys.get_unchecked(i);
            test::black_box(x);
            test::black_box(y);
            }
        }
    })
}

#[bench]
fn zip_loop3(b: &mut test::Bencher)
{
    let xs = vec![0; 1024];
    let ys = vec![0; 768];
    let zs = vec![0; 766];

    b.iter(|| {
        let len = ::std::cmp::min(xs.len(), ::std::cmp::min(ys.len(), zs.len()));
        for i in 0..len {
            unsafe {
            let x = *xs.get_unchecked(i);
            let y = *ys.get_unchecked(i);
            let z = *zs.get_unchecked(i);
            test::black_box(x);
            test::black_box(y);
            test::black_box(z);
            }
        }
    })
}
