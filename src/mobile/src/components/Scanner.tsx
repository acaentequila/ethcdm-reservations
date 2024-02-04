// import { useCanister } from '@connect2ic/react';
import { QrScanner } from '@yudiel/react-qr-scanner';

const Scanner = () => {
	// const [backend] = useCanister('backend')

	// const onDecode = async (decode: any) => {
	// 	// const result = JSON.parse(decode)
	// 	// validate reservation
	// 	// await backend.validateReservation(result.id, result.password)
	// 	// alert(`Valid QR code detected! Payload: ${decode}`)
	// }

	return <QrScanner
		containerStyle={{ height: '100%', width: '100%' }}
		onDecode={(result) => alert(result)}
		// onDecode={onDecode}
		onError={(error) => console.log(error?.message)}
	/>
};

export default Scanner
