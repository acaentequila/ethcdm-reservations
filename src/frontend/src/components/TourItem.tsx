import { Box, Button, Card, Flex, Text } from "theme-ui"
import { Tour } from "../types/tour"
import { useState } from "react"
import ReserveModal from "./ReserveModal"

interface IProps {
	tour: Tour
}
const TourItem: React.FC<IProps> = ({ tour }) => {
	const [modalOpen, setModalOpen] = useState(false)

	return <Card>
		<Flex sx={{ justifyContent: 'space-between' }}>
			<Box>
				<h3>{tour.title}</h3>
				<Text>{tour.price}</Text>
			</Box>
			<Box>
				<Button onClick={() => setModalOpen(true)}>Reserve</Button>
			</Box>
		</Flex>
		<ReserveModal isOpen={modalOpen} onRequestClose={() => setModalOpen(false)} tour={tour}></ReserveModal>
	</Card>
}

export default TourItem

