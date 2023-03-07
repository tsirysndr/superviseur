import { FC, useEffect, useRef, useState } from "react";
import Graph from "react-graph-vis";
import { Service } from "../../Types/Service";

const options = {
  layout: {
    hierarchical: true,
  },
  edges: {
    color: "#000000",
  },
  nodes: {
    shape: "box",
    borderWidth: 2,
    borderWidthSelected: 3,
    font: {
      multi: true,
      size: 12.5,
      color: "#292828ac",
      bold: {
        mod: "",
        size: 14,
        color: "#000",
        vadjust: 3,
      },
    },
    color: {
      border: "#b03aff",
      background: "#fff",
      highlight: {
        border: "#b03aff",
        background: "#fff",
      },
    },
    widthConstraint: {
      minimum: 100,
    },
    heightConstraint: {
      minimum: 40,
    },
    shapeProperties: {
      borderRadius: 2,
    },
  },
};

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
  const createNode = (x: number, y: number) => {
    setState(({ graph: { nodes, edges }, counter, ...rest }) => {
      const id = counter + 1;
      const from = Math.floor(Math.random() * (counter - 1)) + 1;
      return {
        graph: {
          nodes: [...nodes],
          edges: [...edges, { from, to: id }],
        },
        counter: id,
        ...rest,
      };
    });
  };

  const [state, setState] = useState({
    counter: 5,
    graph: {
      nodes: [
        { id: 1, label: "Running\n<b>Service A</b>" },
        { id: 2, label: "Running\n<b>Service B</b>" },
        { id: 3, label: "Running\n<b>Service C</b>" },
        { id: 4, label: "Running\n<b>Service D</b>" },
        { id: 5, label: "Running\n<b>Service E</b>" },
        { id: 6, label: "Running\n<b>Service F</b>" },
      ],
      edges: [
        { from: 1, to: 2 },
        { from: 1, to: 3 },
        { from: 1, to: 4 },
        { from: 2, to: 5 },
      ],
    },
    events: {
      select: ({ nodes, edges }: any) => {
        console.log("Selected nodes:");
        console.log(nodes);
        console.log("Selected edges:");
        console.log(edges);
        //  alert("Selected node: " + nodes);
      },
      doubleClick: ({ pointer: { canvas } }: any) => {
        createNode(canvas.x, canvas.y);
      },
    },
  });
  const { graph, events } = state;

  return (
    <div style={{ height: "100vh", width: "100vw" }}>
      <Graph
        graph={graph}
        options={options}
        events={events}
        style={{ height: "640px" }}
      />
    </div>
  );
};

export default ServicesGraph;
