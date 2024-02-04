import { SHA256 } from 'crypto-js'

export const createPassword = () => {
	const password = crypto.randomUUID()
	const hashed_password = SHA256(password)

	return [
		password,
		hashed_password
	]
}
