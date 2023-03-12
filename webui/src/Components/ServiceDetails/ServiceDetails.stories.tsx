import { ComponentMeta, ComponentStory } from "@storybook/react";
import ServiceDetails, { ServiceDetailsProps } from "./ServiceDetails";

export default {
  title: "Components/ServiceDetails",
  component: ServiceDetails,
  argTypes: {},
} as ComponentMeta<typeof ServiceDetails>;

const Template: ComponentStory<typeof ServiceDetails> = (
  args: ServiceDetailsProps
) => <ServiceDetails {...args} />;

export const Default = Template.bind({});

Default.args = {
  nodes: [
    {
      id: "d548aa8c-2f30-4e53-b938-a4db74de6f58",
      label: "Running\n<b>Service A</b>",
    },
  ],
  selectedNode: "d548aa8c-2f30-4e53-b938-a4db74de6f58",
};
