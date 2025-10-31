import { Link } from 'react-router-dom'

export default function JobCard({ id, title, company, location }:{
  id:string; title:string; company:string; location:string;
}) {
  return (
    <div className="border rounded p-4">
      <h3 className="font-semibold">{title}</h3>
      <p>{company} — {location}</p>
      <Link to={`/jobs/${id}`} className="underline text-sm">View</Link>
    </div>
  )
}
