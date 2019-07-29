import 'package:flutter/material.dart';

class ContextMenuDemo extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return ContextMenuArea(
      items: [
        PopupMenuItem(
          child: ListTile(
            leading: Icon(Icons.settings_applications),
            title: Text('Violin'),
          ),
        ),
        PopupMenuItem(
          child: ListTile(
            leading: Icon(Icons.beenhere),
            title: Text('Viola'),
          ),
        ),
        PopupMenuItem(
          child: ListTile(
            leading: Icon(Icons.gamepad),
            title: Text('Cello'),
          ),
        ),
        PopupMenuItem(
          child: ListTile(
            leading: Icon(Icons.local_gas_station),
            title: Text('Piano'),
          ),
        ),
      ],
      child: Container(
        color: Colors.transparent,
        child: Center(
          child: Text('Right click to show context menu'),
        ),
      )
    );
  }
}

class ContextMenuArea<T> extends StatelessWidget {
  ContextMenuArea({List<PopupMenuEntry<T>> items, Widget child}): child = child, items = items;

  final List<PopupMenuEntry<T>> items;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Listener(
      onPointerDown: (PointerDownEvent evt) {
        if (evt.buttons == 2) {
          var pos = evt.position;

          double width = MediaQuery.of(context).size.width;
          double height = MediaQuery.of(context).size.height;

          // RenderBox box = context.findRenderObject();
          showMenu<T>(
            context: context,
            position: RelativeRect.fromLTRB(pos.dx, pos.dy, width, height),// size.width, size.height),
            items: items,
          );
        }
      },
      child: child,
    );
  }
}
