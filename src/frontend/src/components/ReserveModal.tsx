import { Box, Button, Card, Grid, Input, Label, Text } from "theme-ui"
import Modal from 'react-modal'
import { useCanister } from "@connect2ic/react"
import { Tour } from "../types/tour"
import { FormEvent, useState } from "react"
import { createPassword } from "../utils/passwordManager"
import { createQrCode } from "../utils/qrCode"

interface IProps {
	isOpen: boolean,
	onRequestClose: () => void,
	tour: Tour
}
const ReserveModal: React.FC<IProps> = ({ tour, isOpen, onRequestClose }) => {
	const [backend] = useCanister('backend')
	const [name, setName] = useState<string>('')
	const [persons, setPersons] = useState<number>(1)
	const [date, setDate] = useState<string>('')

	const handleSubmit = async (e: FormEvent) => {
		e.preventDefault()

		// create password
		const [password, hashed_password] = createPassword()


		const id = await backend.createReservation(tour.owner, name, persons, date, hashed_password)

		// create the qr code
		const qrCode = createQrCode({ password })

		// render qr code
		console.log(qrCode, id)
	}

	return <Modal isOpen={isOpen} onRequestClose={onRequestClose}>
		<Card>
			<Grid>
				<Box>{tour.title}</Box>
				<Box>{tour.price}</Box>
				<Box as="form" onSubmit={handleSubmit}>
					<Label>
						<Text>Name</Text>
						<Input onInput={(e) => setName(e.currentTarget.value)}></Input>
					</Label>
					<Label>
						<Text>Persons</Text>
						<Input type="number" onInput={(e) => setPersons(parseInt(e.currentTarget.value))}></Input>
					</Label>
					<Label>
						<Text>Date</Text>
						<Input type="date" onInput={(e) => setDate(e.currentTarget.value)}></Input>
					</Label>



					<Button type="submit">Reserve</Button>
				</Box>
			</Grid>
		</Card>
	</Modal>
}

export default ReserveModal
