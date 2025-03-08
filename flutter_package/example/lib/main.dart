import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:rinf/rinf.dart';
import 'generated.dart';

void main() async {
  await initializeRust(assignRustSignal);
  runApp(MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});
  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  /// This `AppLifecycleListener` is responsible for the
  /// graceful shutdown of the async runtime in Rust.
  /// If you don't care about
  /// properly dropping Rust objects before shutdown,
  /// creating this listener is not necessary.
  late final AppLifecycleListener _listener;

  @override
  void initState() {
    super.initState();
    _listener = AppLifecycleListener(
      onExitRequested: () async {
        finalizeRust(); // This line shuts down the async Rust runtime.
        return AppExitResponse.exit;
      },
    );
  }

  @override
  void dispose() {
    _listener.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Rinf Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.blueGrey,
          brightness: MediaQuery.platformBrightnessOf(context),
        ),
        useMaterial3: true,
      ),
      home: MyHomePage(),
    );
  }
}

class MyHomePage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(child: MyColumn()),
      floatingActionButton: FloatingActionButton(
        onPressed: () async {
          // The `sendSignalToRust` method is generated
          // from a marked Protobuf message.
          // TODO: Update comments like this
          SampleNumberInput(
            letter: 'HELLO FROM DART!',
            dummyOne: 25,
            dummyTwo: SampleSchema(
              sampleFieldOne: true,
              sampleFieldTwo: false,
            ),
            dummyThree: [4, 5, 6],
          ).sendSignalToRust();
        },
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}

class MyColumn extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final children = [
      // `StreamBuilder` listens to a stream
      // and rebuilds the widget accordingly.
      StreamBuilder(
        stream: SampleFractal.rustSignalStream,
        builder: (context, snapshot) {
          final rustSignal = snapshot.data;
          if (rustSignal == null) {
            return Container(
              margin: const EdgeInsets.all(20),
              width: 256,
              height: 256,
              decoration: BoxDecoration(
                borderRadius: BorderRadius.circular(24.0),
                color: Colors.black,
              ),
            );
          }
          final imageData = rustSignal.binary;
          return Container(
            margin: const EdgeInsets.all(20),
            width: 256,
            height: 256,
            child: ClipRRect(
              borderRadius: BorderRadius.circular(24.0),
              child: FittedBox(
                fit: BoxFit.contain,
                child: Image.memory(
                  imageData,
                  width: 256,
                  height: 256,
                  gaplessPlayback: true,
                ),
              ),
            ),
          );
        },
      ),
      StreamBuilder(
        // This stream is generated from a marked Protobuf message.
        stream: SampleNumberOutput.rustSignalStream,
        builder: (context, snapshot) {
          final rustSignal = snapshot.data;
          // If the app has just started and widget is built
          // without receiving a Rust signal,
          // the snapshot data will be null.
          // It's when the widget is being built for the first time.
          if (rustSignal == null) {
            // Return the initial widget if the snapshot data is null.
            return Text('Initial value 0');
          }
          final sampleNumberOutput = rustSignal.message;
          final currentNumber = sampleNumberOutput.currentNumber;
          return Text('Current value is $currentNumber');
        },
      ),
    ];
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: children,
    );
  }
}
