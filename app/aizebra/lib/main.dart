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

typedef LastExpression = ({
  String expression,
  double x,
  double res,
  int iExpression,
  bool visible
});

class _MyHomePageState extends State<MyHomePage> {
  TextEditingController controllerExpr = TextEditingController(text: "sin(x)");
  TextEditingController controllerX = TextEditingController(text: "20");
  TextEditingController controllerNewParam = TextEditingController(text: "a");
  double? res;
  String? error;
  int indexE = -1;
  List<LastExpression> last = [];
  List<LastExpression> parameters = [];
  bool hasHistoric = true;
  bool isCalc = true;
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
        last.insert(0, (
          expression: controllerExpr.text,
          x: x,
          res: res!,
          iExpression: indexE,
          visible: true
        ));
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: fullscreen
          ? null
          : AppBar(
              backgroundColor: Theme.of(context).colorScheme.inversePrimary,
              title: Text("AIzebra"),
            ),
      body: Center(
        child: Padding(
          padding: EdgeInsetsGeometry.all(fullscreen ? 0.0 : 20.0),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: <Widget>[
              !isCalc
                  ? SizedBox()
                  : Card(
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
              SizedBox(height: !isCalc ? 0 : 100),
              fullscreen
                  ? SizedBox()
                  : Row(
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
              SizedBox(height: !isCalc ? 0 : 30),
              !isCalc
                  ? SizedBox(height: fullscreen ? 0 : 20)
                  : Row(
                      children: [
                        SizedBox(width: 100, child: Text("X =")),
                        SizedBox(
                          width: 100,
                          child: TextField(
                            controller: controllerX,
                            textAlign: TextAlign.center,
                          ),
                        ),
                      ],
                    ),
              fullscreen
                  ? SizedBox()
                  : Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        ElevatedButton(
                          onPressed: () => setState(() => isCalc = !isCalc),
                          child: Text(!isCalc ? "Calc" : "Graph"),
                        ),
                        isCalc
                            ? SizedBox()
                            : ElevatedButton(
                                onPressed: () =>
                                    setState(() => hasHistoric = !hasHistoric),
                                child: Text(
                                  hasHistoric ? "Hide history" : "Show history",
                                ),
                              ),
                      ],
                    ),
              !isCalc
                  ? Expanded(
                      child: Row(
                        children: [
                          fullscreen || !hasHistoric
                              ? SizedBox()
                              : SizedBox(
                                  width: 300,
                                  child: Padding(
                                    padding: EdgeInsetsGeometry.symmetric(
                                      vertical: 10.0,
                                      horizontal: 1.0,
                                    ),
                                    child: ListView(
                                      children:
                                          List.generate(
                                            parameters.length,
                                            (index) => ListTile(
                                              title: Dismissible(
                                                direction:
                                                    DismissDirection.horizontal,
                                                onDismissed: (direction) {
                                                  setState(() {
                                                    Parser().evaluate(
                                                      parameters[index]
                                                          .iExpression,
                                                      0,
                                                    );
                                                    //Update ? todo better ?
                                                    last.add(parameters[index]);
                                                    last.removeLast();
                                                    parameters.removeAt(index);
                                                  });
                                                },
                                                key: ValueKey<String>(
                                                  parameters[index].expression + index.toString(),
                                                ),
                                                child: Card(
                                                  child: Column(
                                                    mainAxisAlignment:
                                                        MainAxisAlignment
                                                            .center,
                                                    children: [
                                                      Text(
                                                        parameters[index]
                                                            .expression,
                                                      ),

                                                      Slider(
                                                        value:
                                                            parameters[index].x,
                                                        min: -10.0,
                                                        max: 10.0,
                                                        onChanged: (value) => setState(() {
                                                          Parser().evaluate(
                                                            parameters[index]
                                                                .iExpression,
                                                            value,
                                                          );
                                                          parameters[index] = (
                                                            expression:
                                                                parameters[index]
                                                                    .expression,
                                                            iExpression: parameters[index]
                                                                .iExpression,
                                                            res: 0.0,
                                                            x: value,
                                                            visible: true
                                                          );

                                                          //Update ? todo better ?
                                                          last.add(
                                                            parameters[index],
                                                          );
                                                          last.removeLast();
                                                        }),
                                                      ),
                                                      Text(
                                                        parameters[index].x
                                                            .toStringAsFixed(2),
                                                      ),
                                                    ],
                                                  ),
                                                ),
                                              ),
                                            ),
                                          ).toList() +
                                          [
                                            ListTile(
                                              title:Card(child: 
                                              Padding(padding: EdgeInsetsGeometry.symmetric(vertical: 10, horizontal: 10), child: Row(
                                                children: [
                                                  Expanded(child: 
                                                  TextField(
                                                    textAlign: TextAlign.center,
                                                    controller:
                                                        controllerNewParam,
                                                  ),),
                                                  IconButton(
                                                    onPressed: () {
                                                      setState(() {
                                                        Parser().parse(
                                                          "${controllerNewParam.text}=x",
                                                        );
                                                        parameters.add((
                                                          expression: controllerNewParam.text,
                                                          iExpression: ++indexE,
                                                          res: 0,
                                                          x: 0,
                                                          visible: true
                                                        ));
                                                        Parser().evaluate(
                                                          indexE,
                                                          0,
                                                        );

                                                        //Update ? todo better ?
                                                        last.add(
                                                          parameters.last,
                                                        );
                                                        last.removeLast();
                                                      });
                                                    },
                                                    icon: Icon(Icons.add),
                                                  ),
                                                ],
                                              ),),),
                                            ),
                                            ListTile(title: Divider(),)
                                          ]+
                                          
                                          List.generate(
                                            last.length,
                                            (index) => ListTile(
                                              title: Dismissible(
                                                direction:
                                                    DismissDirection.horizontal,
                                                onDismissed: (direction) {
                                                  setState(() {
                                                    last.removeAt(index);
                                                  });
                                                },
                                                key: ValueKey<LastExpression>(
                                                  last[index],
                                                ),
                                                child: Card(
                                                  child: SizedBox(
                                                    height: 50,
                                                    child: Row(
                                                      mainAxisAlignment:
                                                          MainAxisAlignment
                                                              .start,
                                                      children: [
                                                        SizedBox(width: 20),
                                                        SizedBox(
                                                          width: 20,
                                                          height: 20,
                                                          child: Container(
                                                            color:
                                                                colors[last[index]
                                                                        .iExpression %
                                                                    colors
                                                                        .length],
                                                          ),
                                                        ),
                                                        SizedBox(width: 20),
                                                        Text(
                                                          last[index]
                                                              .expression,
                                                        ),
                                                        Spacer(),
                                                        IconButton(onPressed: () => setState(() {
                                                          last[index]
                                                              = (
                                                            expression:
                                                                last[index]
                                                              .expression,
                                                            iExpression: last[index]
                                                              .iExpression,
                                                            res: last[index]
                                                              .res,
                                                            x: last[index]
                                                              .x,
                                                            visible: !last[index]
                                                              .visible
                                                          );
                                                        }), icon: Icon(last[index].visible ? Icons.visibility : Icons.visibility_off))
                                                      ],
                                                    ),
                                                  ),
                                                ),
                                              ),
                                            ),
                                          ).toList() 
                                          
                                    ),
                                  ),
                                ),
                          Expanded(
                            child: ClipRRect(
                              child: ZoomableCustomWidget(
                                last.map((e) => e.visible ? e.iExpression : -1).where((element) => element >= 0).toList(),
                                () {
                                  setState(() {
                                    fullscreen = !fullscreen;
                                  });
                                },
                              ),
                            ),
                          ),
                        ],
                      ),
                    )
                  : Expanded(
                      child: Padding(
                        padding: EdgeInsetsGeometry.symmetric(
                          vertical: 10.0,
                          horizontal: 20.0,
                        ),
                        child: ListView(
                          children: List.generate(
                            last.length,
                            (index) => ListTile(
                              title: Dismissible(
                                direction: DismissDirection.horizontal,
                                onDismissed: (direction) {
                                  setState(() {
                                    last.removeAt(index);
                                  });
                                },
                                key: ValueKey<LastExpression>(last[index]),
                                child: Card(
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
                          ).toList(),
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
