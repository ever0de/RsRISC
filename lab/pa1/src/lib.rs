//! # Run-length Encoding
//!
//! ## What is Run-length Encoding?
//!
//! A form of lossless data compression in which runs of data are stored as single data value and count
//! - Runs of data: sequence in which the same data value occurs in many consecutive data elements
//!
//! ## Simplified Run-length Encoding
//!
//! 1. Consider the input is a sequence of binary digits
//!     - Just count runs that consist of all 0’s or all 1’s
//!
//! 2. We repeatedly store the length of the run with all 0’s and then with all 1’s
//!     - No need to store data value
//!
//! 3. We allocate three bits to represent the length of each run
//!
//! ## Steps for Simplified Run-length Encoding
//!
//! 1. Find a run with all 0’s in the input. Then, emit the three bits that correspond to the length
//! 2. Find a run with all 1’s in the input. Then, emit the three bits that correspond to the length
//! 3. Repeat step 1 and 2 until the end of the input
//!
//! Encoded data (3 * n bits) // Padding (0~7 bits)
//! -> Byte aligned
//!
//! ## Simplified Run-length Encoding Format
//!
//! - You should write the output in the following format
//! - Output consists of encoded data and a padding
//! - Padding is used to make the final output aligned to bytes
//!
//! ## References
//!
//! <http://csl.snu.ac.kr/courses/4190.308/2021-2/lab1.pdf>

pub mod rle;

pub trait Encoder {
    /// ### Specification
    ///
    /// ```c
    /// int encode(const char* const src, const int srclen, char* const dst, const int dstlen);
    /// ```
    ///
    /// 1. src points to the memory address of the input data
    /// 2. srclen is the length of input data (in bytes)
    /// 3. dst points to the memory address for encoded result
    /// 4. dstlen is the length of allocated space for result
    /// 5. it returns the length of the output (in bytes)
    ///     - if the length of output is bigger than dstlen, return -1 (in this case, contents of the output is ignored)
    ///     - if srclen is zero, return zero
    ///
    /// ### Restrictions
    ///
    /// 1. Contents of the memory outside of the allocated buffer should not be corrupted
    /// 2. You are not allowed to use any array
    /// 3. You are not allowed to use any library functions
    /// 4. Your solution should finish within a reasonable time
    fn encode(
        &self,
        src_pointer: *mut i32,
        src_len: usize,
        dst_pointer: *mut i32,
        dst_len: usize,
    ) -> i8;
}

pub trait Decoder {
    /// ### Spcification
    ///
    /// ```c
    /// int decode(const char* const src, const int srclen, char* const dst, const int dstlen);
    /// ```
    ///
    /// 1. src points to the memory address of the input encoded data
    /// 2. srclen is the length of input data (in bytes)
    /// 3. dst points to the memory address for decoded result
    /// 4. dstlen is the length of allocated space for result
    /// 5. it returns the length of the output (in bytes)
    ///     - if the length of output is bigger than dstlen, return -1 (In this case, contents of the output is ignored)
    ///     - if srclen is zero, return zero
    ///
    /// ### Restrictions
    ///
    /// 1. Contents of the memory outside of the allocated buffer should not be corrupted
    /// 2. You are not allowed to use any array
    /// 3. You are not allowed to use any library functions
    /// 4. Your solution should finish within a reasonable time
    fn decode(
        &self,
        src_pointer: *mut i32,
        src_len: usize,
        dst_pointer: *mut i32,
        dst_len: usize,
    ) -> i8;
}
