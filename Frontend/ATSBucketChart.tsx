export default function ATSBucketChart({ scores }:{ scores:number[] }) {
  const buckets = [0,20,40,60,80]
  const counts = buckets.map((b)=> scores.filter(s=> s>=b && s<(b+20)).length)
  return (
    <div className="p-4 border rounded">
      <h3 className="font-semibold mb-2">ATS Score Buckets</h3>
      <div className="flex gap-2 items-end h-24">
        {counts.map((c,i)=>(
          <div key={i} className="bg-gray-300 w-8" style={{height:`${10*c}px`}} title={`${buckets[i]}-${buckets[i]+19}`}/>
        ))}
      </div>
    </div>
  )
}
