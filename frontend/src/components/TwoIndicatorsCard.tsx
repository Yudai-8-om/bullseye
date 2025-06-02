interface MetricsCardWithTwoIndicatorsProps {
  title: string;
  value: string;
  firstIndicatorTitle: string;
  firstIndicator: string;
  firstClassName: string;
  secondIndicatorTitle: string;
  secondIndicator: string;
  secondClassName: string;
}

function MetricsCardWithTwoIndicators(
  props: MetricsCardWithTwoIndicatorsProps
) {
  const {
    title,
    value,
    firstIndicatorTitle,
    firstIndicator,
    firstClassName,
    secondIndicatorTitle,
    secondIndicator,
    secondClassName,
  } = props;
  return (
    <div className="pl-4 pr-1 pt-1.5 ">
      <header className="flex font-semibold text-sm">{title}</header>
      <div className="flex items-center">
        <div className="text-2xl font-bold">{value}</div>
        <div className="px-3 flex flex-col w-full">
          <div className="flex items-center justify-between mb-1">
            <div className="text-xs">{firstIndicatorTitle}</div>
            <div className={firstClassName}>{firstIndicator}</div>
          </div>
          <div className="flex items-center justify-between">
            <div className="text-xs">{secondIndicatorTitle}</div>
            <div className={secondClassName}>{secondIndicator}</div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default MetricsCardWithTwoIndicators;
