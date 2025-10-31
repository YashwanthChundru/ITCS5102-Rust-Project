export default function ScoreOverview({ avg }:{ avg:number }) {
  return (
    <div className="p-4 border rounded">
      <h3 className="font-semibold mb-2">Average ATS Score</h3>
      <div className="text-2xl">{avg.toFixed(1)}%</div>
    </div>
  )
}
