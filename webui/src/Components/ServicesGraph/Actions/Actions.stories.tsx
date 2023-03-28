import { ComponentMeta, ComponentStory } from "@storybook/react";
import Actions, { ActionsProps } from "./Actions";

export default {
  title: "Components/ServicesGraph/Actions",
  component: Actions,
  argTypes: {
    onStart: { action: "onStart" },
    onStop: { action: "onStop" },
    onRestart: { action: "onRestart" },
    allServicesAreRunning: { control: "boolean" },
    starting: { control: "boolean" },
    stopping: { control: "boolean" },
  },
} as ComponentMeta<typeof Actions>;

const Template: ComponentStory<typeof Actions> = (args: ActionsProps) => (
  <Actions {...args} />
);

export const Default = Template.bind({});

Default.args = {
  allServicesAreRunning: true,
  starting: false,
  stopping: false,
};
