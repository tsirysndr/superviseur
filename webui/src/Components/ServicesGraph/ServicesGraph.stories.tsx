import { ComponentMeta, ComponentStory } from "@storybook/react";
import ServicesGraph from "./ServicesGraph";

export default {
  title: "Components/ServicesGraph",
  component: ServicesGraph,
  argTypes: {},
} as ComponentMeta<typeof ServicesGraph>;

const Template: ComponentStory<typeof ServicesGraph> = (args) => (
  <ServicesGraph {...args} />
);

export const Default = Template.bind({});

Default.args = {
  services: [
    {
      id: "service1",
      name: "service1",
      description: "service1 description",
      dependencies: [
        {
          source: "service1",
          target: "service2",
        },
      ],
      status: "running",
      workingDirectory: "/home/user/service1",
      command: "npm start",
    },
    {
      id: "service2",
      name: "service2",
      description: "service2 description",
      dependencies: [],
      status: "running",
      workingDirectory: "/home/user/service1",
      command: "npm start",
    },
  ],
};
