export default function JobTrendChart({ series }:{ series:{date:string,count:number}[] }) {
  return (
    <div className="p-4 border rounded">
      <h3 className="font-semibold mb-2">Applications Over Time</h3>
      <ul className="text-sm list-disc pl-5">
        {series.map((p)=> <li key={p.date}>{p.date}: {p.count}</li>)}
      </ul>
    </div>
  )
}
