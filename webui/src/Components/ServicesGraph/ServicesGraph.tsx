import { Drawer } from "baseui/drawer";
import { FC, useRef, useState } from "react";
import Graph from "react-graph-vis";
import { Edge } from "../../Types/Edge";
import { Node } from "../../Types/Node";
import ServiceDetails from "../ServiceDetails";

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
      color: "#5b5b5bac",
      face: "RockfordSansMedium",
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

export interface ServicesGraphProps {
  nodes: Node[];
  edges: Edge[];
}

const ServicesGraph: FC<ServicesGraphProps> = (props) => {
  const [selectedNode, setSelectedNode] = useState<string | null>(null);
  const [isOpen, setIsOpen] = useState(false);
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
        if (nodes.length === 0) return;
        setSelectedNode(nodes[0]);
        setIsOpen(true);
      },
      doubleClick: ({ pointer: { canvas } }: any) => {
        createNode(canvas.x, canvas.y);
      },
    },
  });
  const { graph, events } = state;
  const graphRef = useRef<any>();

  return (
    <div style={{ height: "calc(100vh - 60px)", width: "100vw" }}>
      <Graph graph={graph} options={options} events={events} ref={graphRef} />
      <Drawer
        isOpen={isOpen}
        autoFocus
        onClose={() => {
          setIsOpen(false);
          graphRef.current.Network.unselectAll();
        }}
        overrides={{
          Backdrop: {
            style: ({ $theme }) => ({
              outline: `${$theme.colors.warning200} solid`,
              // backgroundColor: $theme.colors.warning200,
            }),
          },
        }}
      >
        <ServiceDetails nodes={props.nodes} selectedNode={selectedNode} />
      </Drawer>
    </div>
  );
};

ServicesGraph.defaultProps = {
  nodes: [],
  edges: [],
};

export default ServicesGraph;
