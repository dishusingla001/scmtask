include!(concat!(env!("OUT_DIR"), "/opencv/alphamat.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/aruco.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/bgsegm.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/bioinspired.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/calib3d.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/ccalib.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/core.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/cudaarithm.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/cudabgsegm.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/cudacodec.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/cudafeatures2d.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/cudafilters.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/cudaimgproc.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/cudalegacy.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/cudaobjdetect.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/cudaoptflow.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/cudastereo.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/cudawarping.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/cvv.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/dnn.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/dnn_superres.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/dpm.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/face.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/features2d.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/flann.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/freetype.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/fuzzy.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/gapi.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/hdf.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/hfs.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/highgui.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/img_hash.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/imgcodecs.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/imgproc.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/intensity_transform.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/line_descriptor.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/mcc.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/ml.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/objdetect.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/optflow.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/ovis.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/phase_unwrapping.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/photo.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/plot.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/quality.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/rapid.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/rgbd.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/saliency.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/sfm.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/shape.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/signal.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/stereo.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/stitching.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/structured_light.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/superres.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/surface_matching.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/text.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/tracking.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/video.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/videoio.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/videostab.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/viz.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/wechat_qrcode.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/xfeatures2d.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/ximgproc.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/xobjdetect.rs"));
include!(concat!(env!("OUT_DIR"), "/opencv/xphoto.rs"));
pub mod types {
	include!(concat!(env!("OUT_DIR"), "/opencv/types.rs"));
}
#[doc(hidden)]
pub mod sys {
	include!(concat!(env!("OUT_DIR"), "/opencv/sys.rs"));
}
pub mod hub_prelude {
	pub use super::alphamat::prelude::*;
	pub use super::aruco::prelude::*;
	pub use super::bgsegm::prelude::*;
	pub use super::bioinspired::prelude::*;
	pub use super::calib3d::prelude::*;
	pub use super::ccalib::prelude::*;
	pub use super::core::prelude::*;
	pub use super::cudaarithm::prelude::*;
	pub use super::cudabgsegm::prelude::*;
	pub use super::cudacodec::prelude::*;
	pub use super::cudafeatures2d::prelude::*;
	pub use super::cudafilters::prelude::*;
	pub use super::cudaimgproc::prelude::*;
	pub use super::cudalegacy::prelude::*;
	pub use super::cudaobjdetect::prelude::*;
	pub use super::cudaoptflow::prelude::*;
	pub use super::cudastereo::prelude::*;
	pub use super::cudawarping::prelude::*;
	pub use super::cvv::prelude::*;
	pub use super::dnn::prelude::*;
	pub use super::dnn_superres::prelude::*;
	pub use super::dpm::prelude::*;
	pub use super::face::prelude::*;
	pub use super::features2d::prelude::*;
	pub use super::flann::prelude::*;
	pub use super::freetype::prelude::*;
	pub use super::fuzzy::prelude::*;
	pub use super::gapi::prelude::*;
	pub use super::hdf::prelude::*;
	pub use super::hfs::prelude::*;
	pub use super::highgui::prelude::*;
	pub use super::img_hash::prelude::*;
	pub use super::imgcodecs::prelude::*;
	pub use super::imgproc::prelude::*;
	pub use super::intensity_transform::prelude::*;
	pub use super::line_descriptor::prelude::*;
	pub use super::mcc::prelude::*;
	pub use super::ml::prelude::*;
	pub use super::objdetect::prelude::*;
	pub use super::optflow::prelude::*;
	pub use super::ovis::prelude::*;
	pub use super::phase_unwrapping::prelude::*;
	pub use super::photo::prelude::*;
	pub use super::plot::prelude::*;
	pub use super::quality::prelude::*;
	pub use super::rapid::prelude::*;
	pub use super::rgbd::prelude::*;
	pub use super::saliency::prelude::*;
	pub use super::sfm::prelude::*;
	pub use super::shape::prelude::*;
	pub use super::signal::prelude::*;
	pub use super::stereo::prelude::*;
	pub use super::stitching::prelude::*;
	pub use super::structured_light::prelude::*;
	pub use super::superres::prelude::*;
	pub use super::surface_matching::prelude::*;
	pub use super::text::prelude::*;
	pub use super::tracking::prelude::*;
	pub use super::video::prelude::*;
	pub use super::videoio::prelude::*;
	pub use super::videostab::prelude::*;
	pub use super::viz::prelude::*;
	pub use super::wechat_qrcode::prelude::*;
	pub use super::xfeatures2d::prelude::*;
	pub use super::ximgproc::prelude::*;
	pub use super::xobjdetect::prelude::*;
	pub use super::xphoto::prelude::*;
}

mod ffi_exports {
	use crate::mod_prelude_sys::*;
	#[no_mangle] unsafe extern "C" fn ocvrs_create_string_0_94_2(s: *const c_char) -> *mut String { unsafe { crate::templ::ocvrs_create_string(s) } }
	#[no_mangle] unsafe extern "C" fn ocvrs_create_byte_string_0_94_2(v: *const u8, len: size_t) -> *mut Vec<u8> { unsafe { crate::templ::ocvrs_create_byte_string(v, len) } }
}
