import React, { createContext, useContext, ReactNode, useState } from "react";
import { Tour } from "../types/tour";

interface TourContextType {
	tour: Tour | null;
	setTour: React.Dispatch<React.SetStateAction<Tour | null>>;
}

const TourContext = createContext<TourContextType | undefined>(undefined);

export const useTour = () => {
	const context = useContext(TourContext);
	if (!context) {
		throw new Error("useTour must be used within a TourProvider");
	}
	return context;
};

interface IProps {
	children: ReactNode;
}

const TourProvider: React.FC<IProps> = ({ children }) => {
	const [tour, setTour] = useState<Tour | null>(null);

	const value: TourContextType = {
		tour,
		setTour,
	};

	return <TourContext.Provider value={value}>
		{children}
	</TourContext.Provider>;
};

export default TourProvider;

