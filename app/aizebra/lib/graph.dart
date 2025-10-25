import 'dart:ui';

import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'dart:math';
import 'rust_lib.dart';
import "package:intl/intl.dart" hide TextDirection;

double log10(num x) => log(x) / ln10;

String formatLargeNumber(double number) {
  NumberFormat formatter = NumberFormat('0.####', 'en_US');
  return formatter.format(number);
}

List<Color> colors = [Colors.blue, Colors.red, Colors.orange, Colors.green, Colors.yellow];
class CurvePainter extends CustomPainter {
  final List<Offset> data = [];
  final List<int> expressions;
  final double scale;
  final Offset focus;

  CurvePainter(this.expressions, this.scale, this.focus);

  void printLabel(Canvas canvas, double label, Offset pos){
      final textPainter = TextPainter(
      text: TextSpan(
        text: formatLargeNumber(label),
        style: TextStyle(color: Colors.black, fontSize: 16),
      ),
      textDirection: TextDirection.ltr,
    )..layout();
    textPainter.paint(canvas, pos.translate(-textPainter.width*0.5, -textPainter.height*0.5));
  }

  Offset toScene(Offset input, Size size){
    return Offset(input.dx *scale + focus.dx + size.width*0.5, input.dy*scale + focus.dy + size.height*0.5);
  }

  @override
  void paint(Canvas canvas, Size size) {
    final rangeXMin = (0-focus.dx-size.width*0.5)/scale;
    final rangeXMax = (size.width-focus.dx-size.width*0.5)/scale;
    final rangeYMin = -(size.height-focus.dy-size.height*0.5)/scale;
    final rangeYMax = -(0-focus.dy-size.height*0.5)/scale;

    final yMinOutScreen = rangeYMin-rangeYMin.abs()*10.0;
    final yMaxOutScreen = rangeYMax+rangeYMax.abs()*10.0;

    final stepSampling = (rangeXMax - rangeXMin)/200.0;
    final stepGrid = pow(10.0, log10((rangeXMax - rangeXMin)/10.0).round()) as double;

    Paint painterBack = Paint();
    painterBack.style = PaintingStyle.stroke;
    painterBack.color = Colors.black12;
    painterBack.strokeWidth = 1.0;

    for (double i = ((rangeXMin/stepGrid).floor()) * stepGrid; i <= rangeXMax; i += stepGrid) {
      canvas.drawLine(toScene(Offset(i, -rangeYMin), size), toScene(Offset(i, -rangeYMax), size), painterBack);
    }
    for (double i = ((rangeYMin/stepGrid).floor()) * stepGrid; i <= rangeYMax; i += stepGrid) {
      canvas.drawLine(toScene(Offset(rangeXMin, -i), size), toScene(Offset(rangeXMax, -i), size), painterBack);
    }

    for (int iExpression in expressions) {
      Paint painterCurve = Paint();
      painterCurve.style = PaintingStyle.stroke;
      painterCurve.color = colors[iExpression % 5];
      painterCurve.strokeWidth = 3.0;
      Offset? lastPoint;
      for (double i = rangeXMin; i <= rangeXMax; i += stepSampling) {
        final y = Parser().evaluate(iExpression, i)!;
        final Offset point = toScene(Offset(i, y.isNaN ? double.nan : -y.clamp(yMinOutScreen, yMaxOutScreen)), size);

        if (lastPoint != null && !lastPoint.dy.isNaN && !y.isNaN) {
          canvas.drawLine(lastPoint, point, painterCurve);
        }

        lastPoint = point;
      }
    }

    Paint painterBlack = Paint();
    painterBlack.style = PaintingStyle.stroke;
    painterBlack.color = Colors.black87;
    painterBlack.strokeWidth = 3.0;
    canvas.drawLine(toScene(Offset(0.0, -rangeYMin), size), toScene(Offset(0.0, -rangeYMax), size), painterBlack);
    canvas.drawLine(toScene(Offset(rangeXMin, -0.0), size), toScene(Offset(rangeXMax, -0.0), size), painterBlack);

    for (double i = ((rangeXMin/stepGrid).floor()) * stepGrid; i <= ((rangeXMax/stepGrid).floor()) * stepGrid; i += stepGrid) {
      if(i.abs() >= stepGrid*0.5){
        printLabel(canvas, i, Offset((i)*scale+focus.dx + size.width*0.5, (0*scale + focus.dy + size.height*0.5+10).clamp(10, size.height-10)));
      }
    }
    for (double i = ((rangeYMin/stepGrid).floor()) * stepGrid; i <= ((rangeYMax/stepGrid).floor()) * stepGrid; i += stepGrid) {
      if(i.abs() >= stepGrid*0.5){
        printLabel(canvas, i, Offset( (0*scale + focus.dx + size.width*0.5+10).clamp(10, size.width-10), (-i)*scale+focus.dy + size.height*0.5));
      }
    }
  }

  @override
  bool shouldRepaint(covariant CurvePainter oldDelegate) {
    return oldDelegate.expressions != expressions ||
        oldDelegate.scale != scale ||
        oldDelegate.focus != focus;
  }
}

class ZoomableCustomWidget extends StatefulWidget {
  const ZoomableCustomWidget(this.expressions, this.callbackDT, {super.key});
  final List<int> expressions;
  final void Function() callbackDT;

  @override
  State<ZoomableCustomWidget> createState() => _ZoomableCustomWidgetState();
}

class _ZoomableCustomWidgetState extends State<ZoomableCustomWidget> {
  double _currentScale = 100.0;
  double _baseScale = 100.0;
  Offset _currentTrans = Offset(0, 0);

  @override
  Widget build(BuildContext context) {
    return Listener(
      onPointerSignal: (PointerSignalEvent event) {
        if (event is PointerScrollEvent) {
          setState(() {
            final box = context.findRenderObject() as RenderBox;
            final size = box.size;

            const zoomSpeed = 0.001;
            final deltaScale = 1 - event.scrollDelta.dy * zoomSpeed;
            final newScale = (_currentScale * deltaScale).clamp(0.001, 200_000.0);

            final focalPoint = event.localPosition;
            final newTrans = focalPoint - Offset(size.width*0.5, size.height * 0.5) - (focalPoint - Offset(size.width*0.5, size.height * 0.5) - _currentTrans) * (newScale / _currentScale);

            _currentScale = newScale;
            _currentTrans = newTrans;
          });
        }
      },
      child: GestureDetector(
        onScaleStart: (details) {
          _baseScale = _currentScale;
        },
        onDoubleTap:  widget.callbackDT,
        onScaleUpdate: (details) {
          setState(() {
            final box = context.findRenderObject() as RenderBox;
            final size = box.size;

            final deltaScale = details.scale;
            final newScale = (_baseScale * deltaScale).clamp(0.001, 200_000.0);

            final focalPoint = details.localFocalPoint;
            final newTrans = focalPoint - Offset(size.width*0.5, size.height * 0.5) - (focalPoint - Offset(size.width*0.5, size.height * 0.5) - _currentTrans) * (newScale / _currentScale);

            _currentScale = newScale;
            _currentTrans = newTrans + details.focalPointDelta;
          });
        },
        child: CustomPaint(
          painter: CurvePainter(
            widget.expressions,
            _currentScale,
            Offset(_currentTrans.dx, _currentTrans.dy),
          ),
          child: Container(),
          // Or use your custom widget that adjusts based on scale
        ),
      ),
    );
  }
}
