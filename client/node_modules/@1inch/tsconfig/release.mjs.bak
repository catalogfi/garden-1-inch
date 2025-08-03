import fs from 'node:fs';
import { execSync } from 'node:child_process';
import semver from 'semver';

const newVersionParam = process.argv[2]
const getNewVersion = (oldVersion, versionParam) => {
  if (semver.valid(versionParam)) {
    return versionParam
  }

  if (['--major', '--patch', '--minor'].includes(versionParam)) {
    return semver.inc(oldVersion, versionParam.replace('--', ''))
  }

  throw new Error(`Unknown version param ${versionParam}`)
}

const run = () => {
  if (!newVersionParam) {
    throw new Error(
      'Invalid version number. Use example "yarn run release (v1.0.0 | --minor | --major | --patch)"',
    );
  }

  const currentBranch = execSync('git rev-parse --abbrev-ref HEAD')
    .toString()
    .trim();

  if (currentBranch !== 'master') {
    throw new Error(
      'You must be on the "release" branch to master. Current in branch: ' +
      currentBranch,
    );
  }

  const pkg = JSON.parse(fs.readFileSync('package.json', 'utf8'));
  const oldVersion = pkg.version;
  const newVersion = getNewVersion(oldVersion, newVersionParam)

  if (semver.gte(oldVersion, newVersion)) {
    throw new Error('New version must be greater than old version');
  }

  pkg.version = newVersion;
  fs.writeFileSync('package.json', JSON.stringify(pkg, null, 2));

  execSync(`git add package.json`, { stdio: 'inherit' });
  execSync(`git commit -m "version v${newVersion}"`, { stdio: 'inherit' });
  execSync(`git tag v${newVersion}`, { stdio: 'inherit' });
  execSync(`git push origin v${newVersion}`, { stdio: 'inherit' });
  execSync(`git push origin master`, { stdio: 'inherit' });
};

run();
