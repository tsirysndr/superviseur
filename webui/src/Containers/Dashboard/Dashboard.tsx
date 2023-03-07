import { FC } from "react";
import Navbar from "../../Components/Navbar";
import ServicesGraph from "../../Components/ServicesGraph";
import { ServicesGraphProps } from "../../Components/ServicesGraph/ServicesGraph";

export interface DashboardProps {
  services?: ServicesGraphProps;
}

const Dashboard: FC<DashboardProps> = ({ services }) => {
  return (
    <div>
      <Navbar />
      {services && <ServicesGraph {...services} />}
    </div>
  );
};

Dashboard.defaultProps = {
  services: {
    nodes: [],
    edges: [],
  },
};

export default Dashboard;
