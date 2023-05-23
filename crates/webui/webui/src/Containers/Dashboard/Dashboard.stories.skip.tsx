import { ComponentMeta, ComponentStory } from "@storybook/react";
import Dashboard, { DashboardProps } from "./Dashboard";

export default {
  title: "Containers/Dashboard",
  component: Dashboard,
  argTypes: {},
} as ComponentMeta<typeof Dashboard>;

const Template: ComponentStory<typeof Dashboard> = (args: DashboardProps) => (
  <Dashboard {...args} />
);

export const Default = Template.bind({});

Default.args = {};
