import { FC } from "react";

const Background: FC = (props) => (
  <svg {...props} width="100%" height="100%">
    <pattern
      id="a"
      x={0}
      y={0}
      width={25}
      height={25}
      patternUnits="userSpaceOnUse"
      patternContentUnits="userSpaceOnUse"
      fill="#a0cfe8"
    >
      <circle cx={10} cy={10} r={1.2} />
    </pattern>
    <rect width="100%" height="100%" fill="url(#a)" />
  </svg>
);

export default Background;
