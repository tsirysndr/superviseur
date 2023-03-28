import { ComponentMeta, ComponentStory } from "@storybook/react";
import { statuses } from "../../Mocks/ServiceStatuses";
import Status from "./Status";

export default {
  title: "Components/Status",
  component: Status,
  argTypes: {
    onStart: { action: "onStart" },
    onRestart: { action: "onRestart" },
    onStop: { action: "onStop" },
  },
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
};

export const Starting = Template.bind({});
Starting.args = {
  statuses: [
    {
      name: "Active",
      status: "Starting",
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
};

export const Stopping = Template.bind({});
Stopping.args = {
  statuses: [
    {
      name: "Active",
      status: "Stopping",
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
};
