import { ComponentMeta, ComponentStory } from "@storybook/react";
import { variables } from "../../Mocks/Variables";
import Variables from "./Variables";

export default {
  title: "Components/Variables",
  component: Variables,
  argTypes: {
    variables: {
      control: {
        type: "array",
      },
      onAdd: { action: "onAdd" },
      onRemove: { action: "onRemove" },
      onEdit: { action: "onEdit" },
    },
  },
} as ComponentMeta<typeof Variables>;

const Template: ComponentStory<typeof Variables> = (args) => (
  <Variables {...args} />
);

export const Default = Template.bind({});

Default.args = {
  variables,
};

export const Empty = Template.bind({});

Empty.args = {
  variables: [],
};
