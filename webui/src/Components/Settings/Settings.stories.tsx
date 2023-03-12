import { ComponentMeta, ComponentStory } from "@storybook/react";
import { settings } from "../../Mocks/Settings";
import Settings from "./Settings";

export default {
  title: "Components/Settings",
  component: Settings,
  argTypes: {},
} as ComponentMeta<typeof Settings>;

const Template: ComponentStory<typeof Settings> = (args) => (
  <Settings {...args} />
);

export const Default = Template.bind({});

Default.args = {
  settings,
};
