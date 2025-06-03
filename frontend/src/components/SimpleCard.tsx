interface SimpleCardProps {
  title: string;
  value: string;
}

function SimpleCard(props: SimpleCardProps) {
  const { title, value } = props;
  return (
    <div className="pl-4 pr-1 pt-1.5">
      <header className="flex font-semibold text-sm">{title}</header>
      <div className="flex justify-center pt-1">
        <div className="text-2xl font-bold">{value}</div>
      </div>
    </div>
  );
}

export default SimpleCard;
