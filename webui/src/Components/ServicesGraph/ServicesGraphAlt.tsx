import { FC, useState } from "react";
import ReactFlow, { Controls, Background } from "reactflow";
import { Service } from "../../Types/Service";

const nodes = [
  {
    id: "a",
    type: "input",
    data: { label: "Service A" },
    position: { x: 0, y: 0 },
    connectable: false,
  },

  {
    id: "b",
    data: { label: "Service B" },
    position: { x: 0, y: 0 },
    connectable: false,
  },
  {
    id: "c",
    data: { label: "Service C" },
    position: { x: 0, y: 0 },
    connectable: false,
  },
  {
    id: "d",
    data: { label: "Service D" },
    position: { x: 0, y: 0 },
    connectable: false,
  },
  {
    id: "g",
    data: { label: "Service G" },
    position: { x: 0, y: 0 },
    connectable: false,
  },
  {
    id: "e",
    data: { label: "Service E" },
    position: { x: 0, y: 0 },
    connectable: false,
  },
  {
    id: "f",
    data: { label: "Service F" },
    position: { x: 0, y: 0 },
    connectable: false,
  },
  {
    id: "h",
    data: { label: "Service H" },
    type: "input",
    position: { x: vw(50), y: 125 },
    connectable: false,
  },

  {
    id: "I",
    data: { label: "Service I" },
    position: { x: vw(50), y: 125 },
    connectable: false,
  },
];

const edges = [
  { id: "ea-b", source: "a", target: "b" },
  { id: "eb-e", source: "b", target: "e" },
  { id: "eb-f", source: "b", target: "f" },
  { id: "ea-c", source: "a", target: "c" },
  { id: "ec-d", source: "c", target: "d" },
  { id: "ec-g", source: "c", target: "g" },
  { id: "eh-i", source: "h", target: "i" },
];

const edgeOptions = {
  type: "smoothstep",
  style: {
    stroke: "black",
  },
};

const connectionLineStyle = { stroke: "black" };

function layout(nodes: any[], edges: any[]) {
  // Initialize the position of each node
  nodes.forEach(function (node) {
    if (!node.position) {
      node.position = { x: 0, y: 0 };
    }
  });

  // Set the position of the root node to the center of the canvas
  var root = nodes.find(function (node: any) {
    return edges.every(function (edge: any) {
      return edge.target !== node.id;
    });
  });
  root.position.x = vw(50);
  root.position.y = 125;

  // display unconnected nodes on the right of the root node
  var unconnectedNodes = nodes.filter(function (node: any) {
    return edges.every(function (edge: any) {
      return edge.target !== node.id;
    });
  });
  var spacing = vw(50) / unconnectedNodes.length + 20;
  for (var i = 0; i < unconnectedNodes.length; i++) {
    var node = unconnectedNodes[i];
    node.position.x = root.position.x + (i + 1) * spacing;
    node.position.y = root.position.y;
  }

  // display subnodes on the left of the root node
  var subNodes = nodes.filter(function (node: any) {
    return edges.some(function (edge: any) {
      return edge.target === node.id;
    });
  });
  spacing = vw(50) / subNodes.length + 20;
  for (var i = 0; i < subNodes.length; i++) {
    var node = subNodes[i];
    node.position.x = root.position.x - (i + 1) * spacing;
    node.position.y = root.position.y;
  }

  // Recursively set the position of each child node relative to its parent
  layoutChildren(root, 1);

  function layoutChildren(node: any, depth: number) {
    var children = getChildren(node);
    var numChildren = children.length;
    var spacing = vw(50) / (numChildren + 1);

    for (var i = 0; i < numChildren; i++) {
      var child = children[i];
      child.position.x = node.position.x - vw(50) / 2 + (i + 1) * spacing;
      child.position.y = node.position.y + 150;
      layoutChildren(child, depth + 1);
    }
  }

  // Helper function to get the children of a node
  function getChildren(node: any) {
    return edges
      .filter(function (edge: any) {
        return node && edge.source === node.id;
      })
      .map(function (edge: any) {
        return nodes.find(function (node: any) {
          return node.id === edge.target;
        });
      });
  }
}

function vh(percent: number) {
  var h = Math.max(
    document.documentElement.clientHeight,
    window.innerHeight || 0
  );
  return (percent * h) / 100;
}

function vw(percent: number) {
  var w = Math.max(
    document.documentElement.clientWidth,
    window.innerWidth || 0
  );
  return (percent * w) / 100;
}

interface ServicesGraphProps {
  services: Service[];
}

const ServicesGraph: FC<ServicesGraphProps> = ({ services }) => {
  layout(nodes, edges);
  return (
    <div style={{ height: "100vh", width: "100vw" }}>
      <ReactFlow
        defaultNodes={nodes}
        defaultEdges={edges}
        defaultEdgeOptions={edgeOptions}
        fitView
        connectionLineStyle={connectionLineStyle}
      >
        <Background />
        <Controls />
      </ReactFlow>
    </div>
  );
};

export default ServicesGraph;
