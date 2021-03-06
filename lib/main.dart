import 'package:flutter/material.dart';
import 'package:workout_ffi/workout_ffi.dart';

void main() {
  runApp(WorkoutApp());
}

class WorkoutApp extends StatelessWidget {
  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Workout',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: ConfigPage(title: 'Workout Configuration'),
    );
  }
}

class ConfigPage extends StatefulWidget {
  ConfigPage({Key key, this.title}) : super(key: key);
  final String title;
  final workout = Workout();

  @override
  _ConfigPageState createState() => _ConfigPageState();
}

class _ConfigPageState extends State<ConfigPage> {
  int _counter = 0;
  int _counterStruct = 0;

  void _incrementCounter() {
    final appState = widget.workout.addStruct(_counterStruct, 1);
    debugPrint("msg: ${appState.msg}");
    debugPrint("err: ${appState.err}");
    setState(() {
      _counter += 1;
      _counterStruct = appState.count;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            Text(
              'You have pushed the button this many times:',
            ),
            Text(
              '$_counter',
              style: Theme.of(context).textTheme.headline4,
            ),
            Text(
              '$_counterStruct',
              style: Theme.of(context).textTheme.headline4,
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _incrementCounter,
        tooltip: 'Increment',
        child: Icon(Icons.add),
      ),
    );
  }
}
