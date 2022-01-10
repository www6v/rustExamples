 
use std::thread;
use std::rc::Rc;
use std::cell::RefCell; 

use std::{
    sync::{Arc, Mutex}
};

#[derive(Clone, Debug)]
struct Developer {
  name: String,
  age: u8,
  lang: Language
}

 #[allow(dead_code)]
 #[derive(Clone, Debug)]
 enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell
 }

pub fn copy_trait_test() {
 let dev = Developer {
    name: "Tyr".to_string(),
    age: 18,
    lang: Language::Rust
 };
 let dev1= dev.clone();
 println!("dev: {:?}, addr of dev name: {:p}", dev, dev.name.as_str());
 println!("dev1: {:?}, addr of dev1 name: {:p}", dev1, dev1.name.as_str())
}


/**
 * 需要注意的是，Copy trait 和 Drop trait 是互斥的，两者不能共存，当你尝试为同一种数
据类型实现 Copy 时，也实现 Drop，编译器就会报错。这其实很好理解：Copy 是按位做
浅拷贝，那么它会默认拷贝的数据没有需要释放的资源；而 Drop 恰恰是为了释放额外的
资源而生的。
 */
/**
 * 对于代码安全来说，内存泄漏危害大？还是 use after free 危害大呢？肯定是后者。Rust
的底线是内存安全，所以两害相权取其轻。
实际上，任何编程语言都无法保证不发生人为的内存泄漏，比如程序在运行时，开发者疏
忽了，对哈希表只添加不删除，就会造成内存泄漏。但 Rust 会保证即使开发者疏忽了，也
不会出现内存安全问题。
 */
use std::{fmt, slice}; 

#[derive(Clone, Copy)] 
struct RawBuffer { 
   ptr: *mut u8, 
   len: usize, 
}

impl From<Vec<u8>> for RawBuffer { 
   fn from(vec: Vec<u8>) -> Self { 
      let slice = vec.into_boxed_slice(); 
      Self {

         len: slice.len(), 
         ptr: Box::into_raw(slice) as *mut u8, 
      } 
   } 
}

// 如果 RawBuffer 实现了 Drop trait，就可以在所有者退出时释放堆内存
// 然后，Drop trait 会跟 Copy trait 冲突，要么不实现 Copy，要么不实现 Drop 
// 如果不实现 Drop，那么就会导致内存泄漏，但它不会对正确性有任何破坏
// 比如不会出现 use after free 这样的问题。
// 你可以试着把下面注释去掉，看看会出什么问题
// impl Drop for RawBuffer { 
//    #[inline] 
//    fn drop(&mut self) { 
//       let data = unsafe { Box::from_raw(slice::from_raw_parts_mut(self.ptr)) }
//       drop(data) 
//    } 
// } 

impl fmt::Debug for RawBuffer { 
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
      let data = self.as_ref(); 
      write!(f, "{:p}: {:?}", self.ptr, data) 
   } 
}

impl AsRef<[u8]> for RawBuffer { 
   fn as_ref(&self) -> &[u8] { 
      unsafe { slice::from_raw_parts(self.ptr, self.len) } 
   } 
}


pub fn drop_trait_test() { 
   let data = vec![1, 2, 3, 4]; 
   let buf: RawBuffer = data.into(); 

   use_buffer(buf); 
   println!("buf: {:?}", buf); 
}

fn use_buffer(buf: RawBuffer) { 
   println!("buf to die: {:?}", buf);  
   drop(buf) 
}


/// 如果在线程间传递 Rc，是无法编译通过的，因为Rc 的实现不支持 Send 和 Sync。
/// Rc 既不是 Send，也不是 Sync
//  fn rc_is_not_send_and_sync() {  
//    let a = Rc::new(1);
//    let b = a.clone();
//    let c = a.clone();
//    thread::spawn(move || {
//        println!("c= {:?}", c);
//    });
// }

/// RefCell 实现了 Send，但没有实现 Sync，所以，看起来是可以工作的
pub fn refcell_is_send() {
   let a = RefCell::new(1);
   thread::spawn(move || {
        println!("a= {:?}", a);
   });
}

// RefCell 现在有多个 Arc 持有它，虽然 Arc 是 Send/Sync，但 RefCell 不是 Sync
//  fn refcell_is_not_sync() {
//    let a = Arc::new(RefCell::new(1));
//    let b = a.clone();
//    let c = a.clone();
//    thread::spawn(move || {
//        println!("c= {:?}", c);
//    });
// }


// Arc<Mutext<T>> 可以多线程共享且修改数据
pub fn arc_mutext_is_send_sync() {
   let a = Arc::new(Mutex::new(1));
   let b = a.clone();
   let c = a.clone();
   let handle = thread::spawn(move || {   
      let mut g = c.lock().unwrap();
      *g += 1; 
   });

   {
      let mut g = b.lock().unwrap();
      *g += 1;
   }
          
   handle.join().unwrap();
   println!("a= {:?}", a);
}

      // fn main() {
      //     arc_mutext_is_send_sync();
      // }      


