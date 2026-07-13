interface ProgressBarProps {
  progress: number;
  speed?: string;
  eta?: string;
  showText?: boolean;
  className?: string;
}

export const ProgressBar = ({ 
  progress, 
  speed, 
  eta,
  showText = true,
  className = ''
}: ProgressBarProps) => {
  return (
    <div className={className}>
      <div className="relative h-2 bg-gray-200 rounded-full overflow-hidden">
        <div
          className="absolute left-0 top-0 h-full bg-blue-500 rounded-full transition-all duration-300 ease-out"
          style={{ width: `${progress}%` }}
        />
      </div>
      {showText && (
        <div className="flex items-center justify-between mt-1">
          <span className="text-xs font-medium text-gray-600">
            {progress.toFixed(1)}%
          </span>
          {speed && (
            <span className="text-xs text-gray-500">{speed}</span>
          )}
          {eta && speed && (
            <span className="text-xs text-gray-400">ETA: {eta}</span>
          )}
        </div>
      )}
    </div>
  );
};
