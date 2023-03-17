import { ComponentMeta, ComponentStory } from "@storybook/react";
import { statuses } from "../../Mocks/ServiceStatuses";
import Status from "./Status";

export default {
  title: "Components/Status",
  component: Status,
  argTypes: {},
} as ComponentMeta<typeof Status>;

const Template: ComponentStory<typeof Status> = (args) => <Status {...args} />;

export const Default = Template.bind({});

Default.args = {
  statuses,
};

export const Empty = Template.bind({});
Empty.args = {
  statuses: [],
};

export const Stopped = Template.bind({});
Stopped.args = {
  statuses: [
    {
      name: "Active",
      status: "Stopped",
    },
    {
      name: "PID",
      status: "1234",
    },
    {
      name: "Command",
      status: "npm start",
    },
    {
      name: "Directory",
      status: "/home/username/website",
    },
    {
      name: "Log",
      status: "/tmp/demo-stdout.log",
    },
    {
      name: "Stderr",
      status: "/tmp/demo-stderr.log",
    },
    {
      name: "AutoRestart",
      status: "true",
    },
    {
      name: "Type",
      status: "exec",
    },
  ],
  onStart: () => {},
  onRestart: () => {},
  onStop: () => {},
};
