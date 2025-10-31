export default function ApplicationTrends({ weekly }:{ weekly:number[] }) {
  return (
    <div className="p-4 border rounded">
      <h3 className="font-semibold mb-2">Weekly Applications</h3>
      <div className="flex gap-2 items-end h-24">
        {weekly.map((c,i)=> <div key={i} className="bg-gray-300 w-6" style={{height:`${8*c}px`}}/>)}
      </div>
    </div>
  )
}
