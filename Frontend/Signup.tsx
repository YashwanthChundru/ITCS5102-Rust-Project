import { useState } from 'react'

export default function Signup() {
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  return (
    <div className="p-6 max-w-md mx-auto">
      <h1 className="text-xl font-semibold mb-4">Create account</h1>
      <form className="space-y-3">
        <input className="border p-2 w-full" placeholder="Email" value={email} onChange={e=>setEmail(e.target.value)} />
        <input className="border p-2 w-full" placeholder="Password" type="password" value={password} onChange={e=>setPassword(e.target.value)} />
        <button className="border px-4 py-2">Sign up</button>
      </form>
    </div>
  )
}
