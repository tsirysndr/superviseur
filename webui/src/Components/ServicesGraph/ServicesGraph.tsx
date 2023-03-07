import { FC, useState } from "react";
import Graph from "react-graph-vis";
import { Edge } from "../../Types/Edge";
import { Node } from "../../Types/Node";

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

interface ServicesGraphProps {
  nodes: Node[];
  edges: Edge[];
}

const ServicesGraph: FC<ServicesGraphProps> = (props) => {
  const createNode = (x: number, y: number) => {
    setState(({ graph: { nodes, edges }, ...rest }) => {
      return {
        graph: {
          nodes: [...nodes],
          edges: [...edges],
        },
        ...rest,
      };
    });
  };

  const [state, setState] = useState({
    graph: {
      nodes: props.nodes,
      edges: props.edges,
    },
    events: {
      select: ({ nodes, edges }: any) => {
        console.log("Selected nodes:");
        console.log(nodes);
        console.log("Selected edges:");
        console.log(edges);
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

ServicesGraph.defaultProps = {
  nodes: [],
  edges: [],
};

export default ServicesGraph;
