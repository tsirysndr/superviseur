import { ComponentMeta, ComponentStory } from "@storybook/react";
import { edges } from "../../Mocks/Edges";
import { nodes } from "../../Mocks/Nodes";
import ServicesGraph, { ServicesGraphProps } from "./ServicesGraph";

export default {
  title: "Components/ServicesGraph",
  component: ServicesGraph,
  argTypes: {},
} as ComponentMeta<typeof ServicesGraph>;

const Template: ComponentStory<typeof ServicesGraph> = (
  args: ServicesGraphProps
) => <ServicesGraph {...args} />;

export const Default = Template.bind({});

Default.args = {
  nodes,
  edges,
};
