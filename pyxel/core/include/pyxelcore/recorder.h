#ifndef PYXELCORE_RECORDER_H_
#define PYXELCORE_RECORDER_H_

#include "pyxelcore/common.h"

namespace pyxelcore {

class Image;

class Recorder {
 public:
  Recorder(int32_t width, int32_t height, int32_t fps);
  ~Recorder();

  void SaveScreenshot();
  void ResetScreenCapture();
  void SaveScreenCapture();

  void Update(const Image* screen_image);

 private:
  int32_t width_;
  int32_t height_;
  int32_t fps_;
  int32_t cur_frame_;
  int32_t start_frame_;
  int32_t frame_count_;
  Image* captured_images_[SCREEN_CAPTURE_COUNT];

  std::string GetFilename() const;
};

}  // namespace pyxelcore

#endif  // PYXELCORE_RECORDER_H_
