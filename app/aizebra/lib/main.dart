import 'package:flutter/material.dart';

import 'rust_lib.dart';
import 'graph.dart';
import 'dart:math';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

typedef LastExpression = ({String expression, double x, double res, int iExpression});

class _MyHomePageState extends State<MyHomePage> {
  TextEditingController controllerExpr = TextEditingController(text: "sin(x)");
  TextEditingController controllerX = TextEditingController(text: "20");
  double? res;
  String? error;
  int indexE = -1;
  List<LastExpression> last = [];
  bool isHistoric = true;
  bool fullscreen = false;


  void _incrementCounter() {
    setState(() {
      if (!Parser().parse(controllerExpr.text)) {
        error = Parser().lastError;
        res = null;
        return;
      }
      final x = double.tryParse(controllerX.text) ?? 0;
      res = Parser().evaluate(++indexE, x);
      if (res == null) {
        error = Parser().lastError;
      } else {
        last.insert(0, (expression: controllerExpr.text, x: x, res: res!, iExpression: indexE));
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: fullscreen ? null : AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text("AIzebra"),
      ),
      body: Center(
        child: Padding(
          padding: EdgeInsetsGeometry.all(fullscreen? 0.0 : 20.0),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: <Widget>[
              !isHistoric ? SizedBox():
              Card(
                child: SizedBox(
                  height: 100,
                  width: 300,
                  child: Center(
                    child: Text(
                      style: TextStyle(fontSize: 16),
                      res == null ? (error ?? "") : res.toString(),
                    ),
                  ),
                ),
              ),
              SizedBox(height: !isHistoric ? 0 : 100),
              fullscreen ? SizedBox() :Row(
                children: [
                  SizedBox(width: 100, child: Text("Expression :")),
                  Expanded(
                    child: TextField(
                      controller: controllerExpr,
                      textAlign: TextAlign.center,
                    ),
                  ),
                  ElevatedButton(
                    onPressed: _incrementCounter,
                    child: Text("Compute"),
                  ),
                ],
              ),
              SizedBox(height: !isHistoric ? 0 : 30),
              fullscreen ? SizedBox() :Row(
                children: [
                  SizedBox(width: 100, child: Text("X =")),
                  SizedBox(width:100,
                    child: TextField(
                      controller: controllerX,
                      textAlign: TextAlign.center,
                    ),
                  ),
                ],
              ),
              fullscreen ? SizedBox() : ElevatedButton(
                onPressed: () => setState(() => isHistoric = !isHistoric,),
                child: Text(isHistoric ? "List" : "Graph"),
              ),
              !isHistoric ? 
              
              Expanded(
                child: ClipRRect(child: ZoomableCustomWidget(last.map((e) => e.iExpression,).toList(), (){setState(() {
                  fullscreen = !fullscreen;
                });})
                )): 
              Expanded(
                child: Padding(
                  padding: EdgeInsetsGeometry.symmetric(
                    vertical: 10.0,
                    horizontal: 20.0,
                  ),
                  child: ListView(
                    children: List.generate(last.length,
                          (index) => ListTile(
                            title: Dismissible(
                              direction:
                                  DismissDirection.horizontal,
                              onDismissed: (direction) {
                                setState(() {
                                  last.removeAt(index);
                                });
                              },
                              key: ValueKey<LastExpression>(last[index]),
                              child : Card(
                              child: SizedBox(
                                height: 50,
                                child: Row(
                                  children: [
                                    Spacer(),
                                    Text(last[index].expression),
                                    Spacer(),
                                    Text(
                                      "evaluated with (x = ${last[index].x}) :",
                                    ),
                                    Spacer(),
                                    Text(
                                      last[index].res.toString().substring(
                                        0,
                                        min(
                                          6,
                                          last[index].res.toString().length,
                                        ),
                                      ),
                                    ),
                                    Spacer(),
                                  ],
                                ),
                              ),
                            ),
                            ),
                          ),
                        )
                        .toList(),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
