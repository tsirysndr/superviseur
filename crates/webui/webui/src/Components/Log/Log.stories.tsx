import { ComponentMeta, ComponentStory } from "@storybook/react";
import { lines } from "../../Mocks/Lines";
import Log from "./Log";

export default {
  title: "Components/Log",
  component: Log,
  argTypes: {
    lines: { control: "array" },
    displayLineNumbers: { control: "boolean" },
  },
} as ComponentMeta<typeof Log>;

const Template: ComponentStory<typeof Log> = (args) => <Log {...args} />;

export const Default = Template.bind({});

Default.args = {
  lines,
};
