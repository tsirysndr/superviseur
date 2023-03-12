import { ComponentMeta, ComponentStory } from "@storybook/react";
import { services } from "../../Mocks/Services";
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

Default.args = {
  services,
};
