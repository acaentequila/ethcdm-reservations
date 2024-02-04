import { createClient } from "@connect2ic/core"
import { Connect2ICProvider } from "@connect2ic/react"
import { InternetIdentity } from '@connect2ic/core/providers/internet-identity';
import { PlugWallet } from "@connect2ic/core/providers/plug-wallet"
import { AstroX } from "@connect2ic/core/providers/astrox"
import "@connect2ic/core/style.css"
import { ReactNode } from "react"
import * as backend from '../../../declarations/backend'

const internetIdentityCanisterId = process.env.INTERNET_IDENTITY_CANISTER_ID

const client = createClient({
	canisters: {
		// @ts-ignore
		backend
	},
	providers: [
		new InternetIdentity({
			providerUrl: `http://192.168.100.75:8000/?canisterId=${internetIdentityCanisterId}`,
			host: window.location.origin
		}),
		new PlugWallet(),
		new AstroX(),
	],
})

console.log({ backend })

interface IProps {
	children: ReactNode
}
const ICProvider: React.FC<IProps> = ({ children }) => (
	<Connect2ICProvider client={client}>
		{children}
	</Connect2ICProvider>
)

export default ICProvider
