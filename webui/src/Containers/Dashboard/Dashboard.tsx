import { FC } from "react";
import Navbar from "../../Components/Navbar";
import ServicesGraph from "../../Components/ServicesGraph";

export interface DashboardProps {}

const Dashboard: FC<DashboardProps> = () => {
  return (
    <div>
      <Navbar />
      <ServicesGraph />
    </div>
  );
};

export default Dashboard;
