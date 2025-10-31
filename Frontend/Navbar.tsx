import { Link } from 'react-router-dom'

export default function Navbar() {
  return (
    <nav className="flex items-center justify-between px-4 py-3 border-b">
      <Link to="/" className="font-semibold">LLM Job Portal</Link>
      <div className="space-x-4">
        <Link to="/ats">ATS</Link>
        <Link to="/login">Login</Link>
        <Link to="/signup">Signup</Link>
      </div>
    </nav>
  )
}
