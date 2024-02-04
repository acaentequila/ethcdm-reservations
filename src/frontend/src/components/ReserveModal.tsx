import { Box, Button, Card, Close, Grid, Input, Label, Text } from "theme-ui"
import Modal from 'react-modal'
import { useCanister } from "@connect2ic/react"
import { Tour } from "../types/tour"
import { FormEvent, useState } from "react"
import { createPassword } from "../utils/passwordManager"
import QRCode from "react-qr-code"

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
	const [qrCode, setQrCode] = useState<string>('')

	const handleSubmit = async (e: FormEvent) => {
		e.preventDefault()

		// create password
		const [password, hashed_password] = createPassword()

		setQrCode(JSON.stringify({ password }))
		alert('done')
		await backend.createReservation(tour.owner, name, persons, date, hashed_password)
	}


	const customStyles = {
		content: {
			top: '50%',
			left: '50%',
			right: 'auto',
			bottom: 'auto',
			marginRight: '-50%',
			transform: 'translate(-50%, -50%)',
		},
	};

	return <Modal style={customStyles} isOpen={isOpen} onRequestClose={onRequestClose}>
		<Card>
			<Grid>
				<Close onClick={onRequestClose} />
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

					{qrCode && <QRCode
						size={256}
						style={{ height: "auto", maxWidth: "100%", width: "100%" }}
						value={qrCode}
						viewBox={`0 0 256 256`}
					/>}
				</Box>
			</Grid>
		</Card>
	</Modal>
}

export default ReserveModal
