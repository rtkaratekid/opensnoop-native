pub mod libbpf;
mod raw_libbpf;

use std::ffi::CString;

fn main() {
  let hash_map_name = CString::new("hashMap name for debugging").unwrap();
  let map_fd = libbpf::bpf_create_map(
    libbpf::BpfMapType::Hash,
    hash_map_name.as_ptr(),
    libbpf::KEY_SIZE,
    libbpf::VAL_T_SIZE,
    /* max_entries */ 10240,
    /* map_flags */ 0,
  );

  println!("{:?}", map_fd);
}
