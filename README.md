# System UI

## 介绍
System UI是一个跨平台的原生小部件库，强调轻量级，他使用系统提供的小部件，因此与系统紧密集成。
在桌面平台基于[libui-ng](https://github.com/libui-ng/libui-ng)、移动端基于[libui-touch](https://github.com/petabyt/libui-touch)两个c库修改并完善。
同时他借鉴了[libui](https://github.com/libui-rs/libui)的代码，对他们的工作表示真诚的感谢。
此项目的设计并不完全等同于libui，因为在未来将增加更多的功能，目前属于开发阶段，很多高级API尚未稳定，因此可能还不适合用于生产环境。
支持的平台包括Windows,MacOS,Linux,iOS,Android等。

## 使用
```shell
cargo add system-ui
```

## 依赖项
- linux基于gtk：
  ```shell
  sudo apt install llvm clang libclang-dev libgtk-3-dev pkg-config
  ```

## 许可证

本项目采用Apache-2.0许可证。请查看项目中的LICENSE文件了解更多信息。

## 贡献

如果您有任何改进意见或想要贡献代码，请随时提交Pull Request或创建Issue。
