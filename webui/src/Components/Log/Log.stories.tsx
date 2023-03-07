import { ComponentMeta, ComponentStory } from "@storybook/react";
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
  lines: [
    "/dev/rdisk1s5s1: fsck_apfs started at Wed Feb 15 20:38:24 2023",
    "/dev/rdisk1s5s1: ** QUICKCHECK ONLY; FILESYSTEM CLEAN",
    "/dev/rdisk1s5s1: fsck_apfs completed at Wed Feb 15 20:38:24 2023",
  ],
};
