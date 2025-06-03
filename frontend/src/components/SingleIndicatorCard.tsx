interface MetricsCardWithSingleIndicatorProps {
  title: string;
  value: string;
  indicatorTitle: string;
  indicator: string;
  className: string;
}

function MetricsCardWithSingleIndicator(
  props: MetricsCardWithSingleIndicatorProps
) {
  const { title, value, indicatorTitle, indicator, className } = props;
  return (
    <div className="pl-3.5 pr-1 pt-1.5 ">
      <header className="flex font-semibold text-sm">{title}</header>
      <div className="flex items-center">
        <div className="text-2xl font-bold">{value}</div>
        <div className="px-3 flex flex-col w-full">
          <div className="flex items-center justify-around mb-1">
            <div className="text-xs">{indicatorTitle}</div>
            <div className={className}>{indicator}</div>
          </div>
        </div>
      </div>
    </div>
  );
}
export default MetricsCardWithSingleIndicator;
