# Introduction

## About Rinf

Rinf combines Flutter's UI ease with Rust's speed, enabling efficient, scalable cross-platform apps.

```{eval-rst}
.. raw:: html
   :file: _templates/icon_pair.html
```

## Design

Rinf expects the application's main logic to be written in Rust, with Flutter solely managing the GUI.

![Rinf design](_static/rinf_design.png)

Rinf uses a two-way, stream-based message-passing mechanism instead of function calls. This decouples the business logic from the UI, ensuring a clear separation of concerns.

It is recommended to use the actor model on the Rust side and tree-based state management techniques such as `InheritedWidget` or `provider` in Flutter for efficient state propagation.

Actors in Rust should hold the state, while Flutter widgets receive only the view data needed to update the UI.

## Cross-Platform Compatibility

Rinf enables seamless development across major platforms:

- ✅ Linux: Tested and supported
- ✅ Android: Tested and supported
- ✅ Windows: Tested and supported
- ✅ macOS: Tested and supported
- ✅ iOS: Tested and supported
- ✅ Web: Tested and supported
