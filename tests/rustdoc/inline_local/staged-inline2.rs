#![crate_name = "std"]
#![feature(staged_api)]
#![stable(feature = "v1", since="1.0.0")]


#[stable(feature = "futures_api", since = "1.36.0")]
//@ has "std/task/index.html" "//span[@class='sub-heading']/span[@class='since']" "1.36.0"
//@ !has - "//span[@class='sub-heading']/span[@class='since']" "1.0.0"
pub mod task {

    #[doc(inline)]
    #[stable(feature = "futures_api", since = "1.36.0")] 
    //@ has "std/task/index.html" "//span[@class='sub-heading']/span[@class='since']" "1.36.0"
    //@ has "std/task/ready/index.html" "//span[@class='sub-heading']/span[@class='since']" "1.64.0"
    pub use core::task::*;
} 

#[stable(feature = "futures_api", since = "1.36.0")] 
//@ has "std/core/index.html" "//span[@class='sub-heading']/span[@class='since']" "1.36.0"
//@ !has - "//span[@class='sub-heading']/span[@class='since']" "1.0.0"
pub mod core {
    #[stable(feature = "futures_api", since = "1.36.0")]
    //@ has "std/core/task/index.html" "//span[@class='sub-heading']/span[@class='since']" "1.36.0"
    pub mod task {

        #[stable(feature = "ready_macro", since = "1.64.0")]
        //@ has "std/core/task/ready/index.html" "//span[@class='sub-heading']/span[@class='since']" "1.64.0"
        pub mod ready {
        }
    }
}